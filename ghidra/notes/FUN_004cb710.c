
undefined4 __fastcall FUN_004cb710(int param_1)

{
  int iVar1;
  int iVar2;
  undefined4 uVar3;
  
  uVar3 = 0;
  *(undefined4 *)(param_1 + 0x6c) = 0;
  iVar1 = FUN_004cc280(param_1);
  if (iVar1 < 1) {
    if (iVar1 < 0) {
      uVar3 = 0xc;
    }
  }
  else {
    iVar2 = FUN_0041b230(*(void **)(param_1 + 0x2c),(uint *)(param_1 + 0x5c),(void *)0x2a,0x4000,
                         0x4000,2);
    if (((*(uint *)(param_1 + 0x5c) & 0xff000000) != 0) && (iVar2 != 0)) {
      *(int *)(param_1 + 0x6c) = iVar1 / iVar2;
    }
    if (0 < *(int *)(param_1 + 0x6c)) {
      if (*(int *)(param_1 + 0x70) < *(int *)(param_1 + 0x6c)) {
        *(int *)(param_1 + 0x6c) = *(int *)(param_1 + 0x70);
      }
      iVar1 = FUN_0047aa90(*(void **)(param_1 + 0x2c),(undefined4 *)(param_1 + 0x58));
      if (iVar1 != 0) {
        iVar2 = *(int *)(iVar1 + 0x84);
        if (*(int *)(param_1 + 0x74) < *(int *)(iVar1 + 0x84)) {
          iVar2 = *(int *)(param_1 + 0x74);
        }
        if (iVar2 < *(int *)(param_1 + 0x6c)) {
          *(int *)(param_1 + 0x6c) = iVar2;
        }
        return 0xe;
      }
    }
  }
  return uVar3;
}

