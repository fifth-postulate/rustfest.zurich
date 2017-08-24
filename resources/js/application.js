(function($, Elm){
    $.deck('.slide');
    $(".clojure").load_snippets();

    var plain = $('#plain-brainbow')[0];
    Elm.Brainbow.embed(plain);
    var word = $('#word-brainbow')[0];
    Elm.Brainbow.embed(word);
    var solve = $('#solve-brainbow')[0];
    Elm.Brainbow.embed(solve);
})(jQuery, Elm);
