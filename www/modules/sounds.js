const shot = new Audio('./sounds/shoot.wav');
shot.type = 'audio/wav';
const player_die = new Audio('./sounds/explosion.wav');
const fleet1 = new Audio('./sounds/fastinvader1.wav');
const fleet2 = new Audio('./sounds/fastinvader2.wav');
const fleet3 = new Audio('./sounds/fastinvader3.wav');
const fleet4 = new Audio('./sounds/fastinvader4.wav');
const invader_die = new Audio('./sounds/invaderkilled.wav');
const ufo_highpitch = new Audio('./sounds/ufo_highpitch.wav');
ufo_highpitch.loop = true;
ufo_highpitch.pause();

export function play_shot(){
    shot.play();
}
export function play_player_die(){
    player_die.play();
}
export function play_invader_die(){
    invader_die.play();
}
export function play_fleet1(){
    fleet1.play();
}
export function play_fleet2(){
    fleet2.play();
}
export function play_fleet3(){
    fleet3.play();
}
export function play_fleet4(){
    fleet4.play();
}
export function play_ufo_highpitch(){
    ufo_highpitch.play();
}
export function stop_ufo_highpitch(){
    ufo_highpitch.pause();
    ufo_highpitch.currentTime = 0;
}
//export {foo};