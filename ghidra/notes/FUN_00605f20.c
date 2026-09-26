
void __fastcall FUN_00605f20(int param_1)

{
  size_t sVar1;
  size_t sVar2;
  
  sVar2 = *(size_t *)(param_1 + 0xb4);
  sVar1 = *(size_t *)(param_1 + 0xb8);
  if (sVar2 == sVar1) {
    return;
  }
  if ((sVar2 == 0xffffffff) && (-1 < (int)sVar1)) {
    sVar2 = *(size_t *)(param_1 + 0xd0);
  }
  else {
    if ((sVar1 == 0xffffffff) && (-1 < (int)sVar2)) {
      FUN_005f3c30((void *)(param_1 + 0x98),sVar2,*(int *)(param_1 + 0xd0));
      *(undefined4 *)(param_1 + 0xcc) = *(undefined4 *)(param_1 + 0xb8);
      goto LAB_00605faa;
    }
    if ((int)sVar2 <= (int)sVar1) {
      FUN_005f3c30((void *)(param_1 + 0x98),sVar2,sVar1);
      *(undefined4 *)(param_1 + 0xcc) = *(undefined4 *)(param_1 + 0xb4);
      goto LAB_00605faa;
    }
  }
  FUN_005f3c30((void *)(param_1 + 0x98),sVar1,sVar2);
  *(undefined4 *)(param_1 + 0xcc) = *(undefined4 *)(param_1 + 0xb8);
LAB_00605faa:
  *(undefined4 *)(param_1 + 0xb8) = 0xffffffff;
  *(undefined4 *)(param_1 + 0xb4) = 0xffffffff;
  return;
}

