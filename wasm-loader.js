module.exports = function(source) {
    // Replace problematic requires with imports
    return source.replace(/require\(['"]util['"]\)/g, 'import("util")');
};
