
undefined4 __fastcall FUN_004cdfd0(int param_1)

{
  int iVar1;
  int iVar2;
  undefined4 uVar3;
  
  uVar3 = 0;
  *(undefined4 *)(param_1 + 0x68) = 0;
  iVar1 = FUN_004cec80(param_1);
  if (iVar1 < 1) {
    if (iVar1 < 0) {
      uVar3 = 7;
    }
  }
  else {
    iVar2 = FUN_0041b230(*(void **)(param_1 + 0x2c),(uint *)(param_1 + 100),(void *)0x2a,0x4000,
                         0x4000,2);
    if (((*(uint *)(param_1 + 100) & 0xff000000) != 0) && (iVar2 != 0)) {
      *(int *)(param_1 + 0x68) = iVar1 / iVar2;
    }
    if (0 < *(int *)(param_1 + 0x68)) {
      if (*(int *)(param_1 + 0x6c) < *(int *)(param_1 + 0x68)) {
        *(int *)(param_1 + 0x68) = *(int *)(param_1 + 0x6c);
      }
      iVar1 = FUN_0047aa90(*(void **)(param_1 + 0x2c),(undefined4 *)(param_1 + 0x60));
      if (iVar1 != 0) {
        iVar2 = *(int *)(iVar1 + 0x84);
        if (*(int *)(param_1 + 0x70) < *(int *)(iVar1 + 0x84)) {
          iVar2 = *(int *)(param_1 + 0x70);
        }
        if (iVar2 < *(int *)(param_1 + 0x68)) {
          *(int *)(param_1 + 0x68) = iVar2;
        }
        return 9;
      }
    }
  }
  return uVar3;
}

