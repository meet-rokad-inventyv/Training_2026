// @ts-check

/**
 * Implement the classes etc. that are needed to solve the
 * exercise in this file. Do not forget to export the entities
 * you defined so they are available for the tests.
 */

export class Size {
  // constructor(){
  //   this.width = 80;
  //   this.height = 60;
  // }
  constructor(width = 80, height = 60) {
    this.width = width;
    this.height = height;
  }
  resize(newWidth, newHeight) {
    this.width = newWidth;
    this.height = newHeight;
  }
}

export class Position {
  constructor(x = 0, y = 0) {
    this.x = x;
    this.y = y;
  }
  move(newX, newY) {
    this.x = newX;
    this.y = newY;
  }
}

export class ProgramWindow {
  constructor(){
    this.screenSize = new Size(800, 600);
    this.size = new Size();
    this.position = new Position();
  }
  resize(newSize) {
      const mw = this.screenSize.width - this.position.x;
      const mh = this.screenSize.height - this.position.y;
      const nw = Math.max(1, Math.min(newSize.width, mw));
      const nh = Math.max(1, Math.min(newSize.height, mh));
      this.size.resize(nw, nh);
  }
  move(newPosition) {
    const mx = this.screenSize.width - this.size.width;
    const my = this.screenSize.height - this.size.height;
    const nx = Math.max(0, Math.min(newPosition.x, mx));
    const ny = Math.max(0, Math.min(newPosition.y, my));
    this.position.move(nx, ny);
  }
}

export function changeWindow(programWindow) {
  programWindow.move(new Position());
  programWindow.resize(new Size(400, 300));
  programWindow.move(new Position(100, 150));
  return programWindow;
}
