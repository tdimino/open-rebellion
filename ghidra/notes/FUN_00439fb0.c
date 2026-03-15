
void __fastcall FUN_00439fb0(int param_1)

{
  undefined4 *puVar1;
  undefined4 uVar2;
  uint uVar3;
  
  puVar1 = (undefined4 *)thunk_FUN_005f5060(param_1 + 0x154);
  uVar3 = 0;
  if ((*(uint *)(param_1 + 0x150) & 0x10000000) == 0) {
    FUN_00439f20(param_1);
  }
  while ((puVar1 != (undefined4 *)0x0 && (uVar3 == 0))) {
    switch(puVar1[7] & 0xf) {
    default:
      uVar3 = 1;
      break;
    case 2:
      uVar3 = *(uint *)(param_1 + 0x150) >> 1 & 1;
      break;
    case 3:
      uVar3 = *(uint *)(param_1 + 0x150) >> 2 & 1;
      break;
    case 4:
      uVar3 = *(uint *)(param_1 + 0x150) >> 4 & 1;
      break;
    case 5:
      uVar3 = *(uint *)(param_1 + 0x150) >> 3 & 1;
      break;
    case 6:
      uVar3 = *(uint *)(param_1 + 0x150) & 1;
    }
    if (uVar3 == 0) {
      puVar1 = (undefined4 *)puVar1[4];
    }
    else {
      FUN_0048a590((void *)(*(int *)(param_1 + 0x144) + 0x6c),(int)puVar1);
      puVar1 = (undefined4 *)FUN_005f54d0((void *)(param_1 + 0x154),puVar1[6]);
      if (puVar1 == (undefined4 *)0x0) break;
      (**(code **)*puVar1)(1);
    }
  }
  uVar2 = thunk_FUN_004fcee0();
  *(undefined4 *)(param_1 + 0x148) = uVar2;
  return;
}

