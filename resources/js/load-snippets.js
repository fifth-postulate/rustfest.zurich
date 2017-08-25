(function($){
    $.fn.load_snippets = function(options){
        console.log("loading snippets");
        var settings = $.extend({
            location: 'resources/snippets/',
            data: 'snippet'
        }, options);
        this.each(function(index, element){
            var url = settings.location + $(element).data(settings.data);
            $.ajax(url).done(function(data){
                console.log(data);
                $(element).val(data);
            });
        });

        return this;
    };
})(jQuery);
