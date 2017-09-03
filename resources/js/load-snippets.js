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
            });
        });

        return this;
    };
})(jQuery, ace);
