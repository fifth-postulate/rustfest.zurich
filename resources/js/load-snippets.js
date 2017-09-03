(function($, ace){
    $.fn.load_snippets = function(options){
        console.log("loading snippets");
        var settings = $.extend({
            location: 'resources/snippets/',
            data: 'snippet'
        }, options);
        this.each(function(index, element){
            var t = $("<textarea>");
            var b = $("<button>"); b.text("Run");
            var r = $("<div>");
            [t, b, r].forEach(function(child){ $(element).append(child); });
            var url = settings.location + $(element).data(settings.data);
            $.ajax(url).done(function(data){
                var editor = ace.edit(t.get()[0]);
                editor.setOptions({
                    maxLines: 20
                });
                editor.setTheme('ace/theme/chrome');
                var session = editor.getSession();
                session.setMode('ace/mode/rust');
                session.setNewLineMode('unix');
                session.setValue(data);
                b.click(function(){
                    r.html('Running...');
                    var program = session.getValue();
                    runProgram(program, function(status, message){
                        if (status === 0) {
                            var lines = message.split(newLineRegex);
                            message = lines.map(function(line) {
                                return escapeHTML(line);
                            }).join('<br />');
                            r.text(message);
                        } else {
                            r.text('Whoops');
                        }
                    });
                });
            });
        });

        return this;
    };
})(jQuery, ace);

function runProgram(program, callback) {
    var req = new XMLHttpRequest();
    var data = JSON.stringify({
        version: "stable",
        optimize: "0",
        code: program
    });

    req.open('POST', "http://localhost:5000/evaluate.json", true);
    req.onload = function(e) {
        if (req.readyState === 4 && req.status === 200) {
            var result = JSON.parse(req.response).result;

            // Need server support to get an accurate version of this.
            var statusCode = SUCCESS;
            if (result.indexOf("error:") !== -1) {
                statusCode = ERROR;
            } else if (result.indexOf("warning:") !== -1) {
                statusCode = WARNING;
            }

            callback(statusCode, result);
        } else {
            callback(false, null);
        }
    };

    req.onerror = function(e) {
        callback(false, null);
    };

    req.setRequestHeader("Content-Type", "application/json");
    req.send(data);
}

var newLineRegex = /(?:\r\n|\r|\n)/g;
var entityMap = {
    "&": "&amp;",
    "<": "&lt;",
    ">": "&gt;",
    '"': '&quot;',
    "'": '&#39;',
    "/": '&#x2F;'
};

function escapeHTML(unsafe) {
    return String(unsafe).replace(/[&<>"'\/]/g, function(s) {
    return entityMap[s];
  });
}

function handleResult(statusCode, message) {
}
