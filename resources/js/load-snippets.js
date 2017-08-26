(function($, ace){
    $.fn.load_snippets = function(options){
        console.log("loading snippets");
        var settings = $.extend({
            location: 'resources/snippets/',
            data: 'snippet'
        }, options);
        this.each(function(index, element){
            var url = settings.location + $(element).data(settings.data);
            $.ajax(url).done(function(data){
                var editor = ace.edit(element);
                editor.setOptions({
                    maxLines: 15
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
