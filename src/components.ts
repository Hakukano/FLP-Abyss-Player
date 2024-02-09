export default class Component {
  content: JQuery<HTMLElement>;

  constructor(content: JQuery<HTMLElement>) {
    this.content = content;
  }

  render(): string {
    return this.content.html();
  }

  reload(): Promise<any> {
    return new Promise((resolve, _) => {
      resolve(null);
    });
  }
}
