
undefined4 FUN_0045ca80(undefined4 param_1,undefined4 param_2,int param_3)

{
  undefined4 uVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  undefined4 local_4;
  
  uVar1 = 0;
  uVar2 = 0;
  uVar3 = 0;
  uVar4 = 0;
  uVar5 = 0;
  local_4 = 0;
  switch(param_1) {
  case 4:
    uVar2 = 0x2a13;
    uVar3 = 0x2a14;
    uVar4 = 0x2a1b;
    uVar5 = 0x2a1c;
    uVar1 = 0x2a23;
    local_4 = 0x2a24;
    break;
  case 8:
    uVar2 = 0x2a15;
    uVar3 = 0x2a16;
    uVar4 = 0x2a1d;
    uVar5 = 0x2a1e;
    uVar1 = 0x2a25;
    local_4 = 0x2a26;
    break;
  case 0x10:
    uVar2 = 0x2a17;
    uVar3 = 0x2a18;
    uVar4 = 0x2a1f;
    uVar5 = 0x2a20;
    break;
  case 0x40:
    uVar2 = 0x2a19;
    uVar3 = 0x2a1a;
    uVar4 = 0x2a21;
    uVar5 = 0x2a22;
  }
  switch(param_2) {
  case 0:
  case 3:
    break;
  case 1:
    if (param_3 != 0) {
      return uVar3;
    }
    return uVar2;
  case 2:
    if (param_3 != 0) {
      return uVar5;
    }
    return uVar4;
  default:
    return 0;
  }
  if (param_3 != 0) {
    return local_4;
  }
  return uVar1;
}

