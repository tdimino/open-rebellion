
undefined4 __fastcall FUN_004d8460(int param_1)

{
  int iVar1;
  int iVar2;
  uint uVar3;
  int iVar4;
  undefined4 uVar5;
  
  uVar5 = 9;
  *(undefined4 *)(param_1 + 0x60) = 0;
  iVar1 = FUN_004da4d0(param_1);
  if (iVar1 < 1) {
    if (iVar1 < 0) {
      uVar5 = 7;
    }
  }
  else {
    iVar2 = FUN_0047aa90(*(void **)(param_1 + 0x2c),(undefined4 *)(param_1 + 0x54));
    iVar4 = param_1;
    if ((iVar2 != 0) && (0 < *(int *)(iVar2 + 0x84))) {
      if ((*(uint *)(iVar2 + 0x28) & 0x40000000) == 0) {
        if ((*(uint *)(iVar2 + 0x28) & 0x8000) == 0) {
          uVar3 = 0x80000;
          iVar4 = 0x40000;
        }
        else {
          uVar3 = 0x20000;
          iVar4 = 0x10000;
        }
      }
      else {
        uVar3 = 0x40000;
        iVar4 = 0x10000;
      }
      iVar4 = FUN_0041b230(*(void **)(param_1 + 0x2c),(uint *)(param_1 + 0x5c),(void *)0x2a,uVar3,
                           iVar4,1);
    }
    if ((((*(uint *)(param_1 + 0x5c) & 0xff000000) != 0) && (iVar4 != 0)) && (iVar4 <= iVar1)) {
      *(undefined4 *)(param_1 + 0x60) = 1;
    }
    if (0 < *(int *)(param_1 + 0x60)) {
      return 0xb;
    }
  }
  return uVar5;
}

