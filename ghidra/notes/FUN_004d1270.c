
undefined4 __fastcall FUN_004d1270(int param_1)

{
  int iVar1;
  int iVar2;
  undefined4 uVar3;
  
  uVar3 = 0;
  *(undefined4 *)(param_1 + 0x48) = 0;
  iVar1 = FUN_004d1df0(param_1);
  if (iVar1 < 1) {
    if (iVar1 < 0) {
      uVar3 = 8;
    }
  }
  else {
    iVar2 = FUN_0041b230(*(void **)(param_1 + 0x2c),(uint *)(param_1 + 100),(void *)0x2a,0x8000,
                         0x4000,2);
    if (((*(uint *)(param_1 + 100) & 0xff000000) != 0) && (iVar2 != 0)) {
      *(int *)(param_1 + 0x48) = iVar1 / iVar2;
    }
    if (0 < *(int *)(param_1 + 0x48)) {
      if (*(int *)(param_1 + 0x4c) < *(int *)(param_1 + 0x48)) {
        *(int *)(param_1 + 0x48) = *(int *)(param_1 + 0x4c);
      }
      iVar1 = FUN_0047aa90(*(void **)(param_1 + 0x2c),(undefined4 *)(param_1 + 0x5c));
      if (iVar1 != 0) {
        iVar2 = *(int *)(iVar1 + 0x84);
        if (*(int *)(param_1 + 0x44) < *(int *)(iVar1 + 0x84)) {
          iVar2 = *(int *)(param_1 + 0x44);
        }
        if (iVar2 < *(int *)(param_1 + 0x48)) {
          *(int *)(param_1 + 0x48) = iVar2;
        }
        return 10;
      }
    }
  }
  return uVar3;
}

