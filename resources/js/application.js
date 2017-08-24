(function($, Elm, ace){
    $.deck('.slide');
    $(".rust")
        .load_snippets()
        .each(function(index, element){
            var editor = ace.edit(element);
            editor.setTheme("ace/theme/chrome");
            editor.getSession().setMode('ace/mode/rust');
        });

    var plain = $('#plain-brainbow')[0];
    Elm.Brainbow.embed(plain);
    var word = $('#word-brainbow')[0];
    Elm.Brainbow.embed(word);
    var solve = $('#solve-brainbow')[0];
    Elm.Brainbow.embed(solve);
})(jQuery, Elm, ace);
