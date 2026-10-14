
undefined4 FUN_005ec6d0(undefined4 param_1,int *param_2,int param_3)

{
  int iVar1;
  int *piVar2;
  void *pvVar3;
  void *this;
  
  iVar1 = FUN_005f5060(param_3);
  if ((iVar1 != 0) && (iVar1 = (**(code **)(*param_2 + 0xc))(), iVar1 != 0)) {
    for (piVar2 = (int *)(**(code **)(*param_2 + 0xc))(); piVar2 != (int *)0x0;
        piVar2 = (int *)(**(code **)(*piVar2 + 4))()) {
      iVar1 = FUN_005f5060(param_3);
      iVar1 = *(int *)(iVar1 + 0x1c);
      pvVar3 = (void *)(**(code **)(*piVar2 + 0xc))();
      this = (void *)FUN_005aaf90();
      FUN_005a8fc0(this,iVar1,pvVar3);
    }
    return 1;
  }
  return 0;
}

