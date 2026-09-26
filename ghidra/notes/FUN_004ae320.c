
undefined4 __thiscall FUN_004ae320(void *this,int param_1,void *param_2,undefined4 *param_3)

{
  int iVar1;
  void *pvVar2;
  int *piVar3;
  int iVar4;
  int iVar5;
  int iVar6;
  undefined4 local_8;
  
  pvVar2 = param_2;
  local_8 = 0;
  *param_3 = 0;
  if ((((param_2 != (void *)0x0) && (param_1 != 0)) &&
      (iVar4 = *(int *)((int)param_2 + 0x3c), iVar4 != 0)) &&
     ((*(byte *)((int)param_2 + 0x30) & 4) != 0)) {
    piVar3 = FUN_00403040(param_2,&param_2);
    iVar1 = *piVar3;
    iVar5 = *(int *)(param_1 + 0x28);
    FUN_00619730();
    iVar6 = 0;
    if (iVar1 != iVar5) {
      iVar1 = *(int *)(*(int *)((int)this + 0xc) + 0x168);
      iVar6 = *(int *)((int)pvVar2 + 0xb4);
      if (*(int *)((int)pvVar2 + 0xb4) < iVar1) {
        iVar6 = iVar1;
      }
    }
    if (((iVar6 <= *(int *)(param_1 + 0x5c)) &&
        (*(int *)(param_1 + 0x68) != *(int *)((int)pvVar2 + 0x68) &&
         -1 < *(int *)(param_1 + 0x68) - *(int *)((int)pvVar2 + 0x68))) &&
       (*(int *)((int)pvVar2 + 0x74) == 0)) {
      iVar1 = *(int *)((int)pvVar2 + 0x90);
      iVar5 = *(int *)((int)pvVar2 + 100) + iVar1;
      iVar4 = ((*(int *)(iVar4 + 0x78) - *(int *)(iVar4 + 0x80)) + 1) / 2;
      if (iVar4 < 1) {
        if ((0 < iVar1) || (0 < *(int *)((int)pvVar2 + 0xcc))) {
          local_8 = 1;
          *param_3 = 0x222;
        }
      }
      else if ((iVar5 != iVar4 && -1 < iVar5 - iVar4) && (0 < iVar1)) {
        *param_3 = 0x220;
        return 1;
      }
    }
  }
  return local_8;
}

