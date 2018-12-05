var input = 325489;
function num2xy(x) {
  if (x === 0) { return [0,0]; }
  var s = Math.floor(Math.sqrt(x));
  var r = Math.floor((s - 1) / 2) + 1;
  a = x - Math.pow((r * 2) - 1, 2);
  var da = (a % (r * 2)) - r + 1;
  var q = Math.floor(a / (r * 2));
  var x,y;
  switch(q) {
      case 0: x = r; y = da; break;
      case 1: y = r; x = -da; break;
      case 2: x = -r; y = -da; break;
      case 3: y = -r; x = da; break;
  }
  return [x,y];
}
var xy = num2xy(input).map(Math.abs);
console.log(xy[0] + xy[1]);

function num2xys(x) { return num2xy(x).join(','); }

var field = {'0,0': 1};

function sumAround(x) {
  var xy = num2xy(x);
  var s = 0;
  for (var dx = -1; dx < 2; dx++) {
    for (var dy = -1; dy < 2; dy++) {
      if (dx === 0 && dy === 0) { continue; }
      var k = (xy[0] + dx) + ',' + (xy[1] + dy);
      s += field[k] || 0;
    }
  }
  return s;
}

for (var i = 1; field[num2xys(i-1)] < input; i++) {
  field[num2xys(i)] = sumAround(i);
}
console.log(field[num2xys(i-1)]);
