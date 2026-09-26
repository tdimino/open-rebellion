
void __fastcall FUN_005d0bb0(int param_1)

{
  int iVar1;
  int iVar2;
  void *this;
  int iVar3;
  void *pvVar4;
  
  iVar3 = 0;
  iVar2 = *(int *)(*(int *)(param_1 + 8) + 0x50) * 4 + 8;
  iVar1 = FUN_005aaf90();
  pvVar4 = *(void **)(iVar1 + iVar2);
  if (*(int *)(param_1 + 0xc) == 0) {
    iVar1 = FUN_005aaf90();
    iVar2 = FUN_005c91e0(*(void **)(iVar1 + iVar2),*(undefined4 *)(param_1 + 8));
    if (iVar2 == 0) {
      for (iVar2 = FUN_005f5060((int)pvVar4 + 0x90); iVar2 != 0; iVar2 = *(int *)(iVar2 + 0x10)) {
        iVar1 = (**(code **)(**(int **)(iVar2 + 0x1c) + 0x20))();
        if ((iVar1 == 1) && (iVar1 = FUN_005c91c0(pvVar4,*(undefined4 *)(iVar2 + 0x1c)), iVar1 != 0)
           ) {
          iVar3 = *(int *)(iVar2 + 0x1c);
          break;
        }
      }
    }
    if (iVar3 != 0) {
      pvVar4 = *(void **)(param_1 + 8);
      *(undefined4 *)(param_1 + 0xc) = 1;
      this = (void *)FUN_005aaf90();
      FUN_005a8fc0(this,iVar3,pvVar4);
    }
  }
  return;
}

