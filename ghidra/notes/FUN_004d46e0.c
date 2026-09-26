
undefined4 __fastcall FUN_004d46e0(int param_1)

{
  int iVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  undefined4 uVar5;
  
  uVar5 = 0;
  *(undefined4 *)(param_1 + 0x60) = 0;
  iVar1 = FUN_004d66a0(param_1);
  if (iVar1 < 1) {
    if (iVar1 < 0) {
      uVar5 = 7;
    }
  }
  else {
    iVar2 = FUN_0047aa90(*(void **)(param_1 + 0x2c),(undefined4 *)(param_1 + 0x54));
    iVar3 = param_1;
    iVar4 = param_1;
    if (iVar2 != 0) {
      *(int *)(param_1 + 0x60) = -*(int *)(iVar2 + 0xe0);
      if ((*(uint *)(iVar2 + 0x28) & 0x83e00000) == 0) {
        iVar3 = 1;
        iVar4 = 2;
      }
      else {
        iVar3 = 0x400;
        iVar4 = 1;
      }
    }
    iVar3 = FUN_0041b230(*(void **)(param_1 + 0x2c),(uint *)(param_1 + 0x5c),(void *)0x28,0x200000,
                         iVar3,iVar4);
    if ((((*(uint *)(param_1 + 0x5c) & 0xff000000) != 0) && (iVar3 != 0)) &&
       (iVar4 = iVar3 * *(int *)(param_1 + 0x60), iVar4 - iVar1 != 0 && iVar1 <= iVar4)) {
      *(int *)(param_1 + 0x60) = iVar1 / iVar3;
    }
    if (0 < *(int *)(param_1 + 0x60)) {
      if (*(int *)(param_1 + 100) < *(int *)(param_1 + 0x60)) {
        *(int *)(param_1 + 0x60) = *(int *)(param_1 + 100);
      }
      FUN_004ece80((undefined4 *)(*(int *)(param_1 + 0x2c) + 0x1c));
      return 0xb;
    }
  }
  return uVar5;
}

