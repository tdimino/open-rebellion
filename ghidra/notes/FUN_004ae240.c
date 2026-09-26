
undefined4 __thiscall FUN_004ae240(void *this,int param_1,void *param_2)

{
  int iVar1;
  int iVar2;
  void *pvVar3;
  int *piVar4;
  int iVar5;
  int iVar6;
  undefined4 local_c;
  
  pvVar3 = param_2;
  iVar2 = param_1;
  iVar6 = 0;
  local_c = 0;
  if ((((param_1 != 0) && (param_2 != (void *)0x0)) &&
      ((*(uint *)((int)param_2 + 0x30) & 0x101) == 0)) &&
     ((*(uint *)((int)param_2 + 0x30) & 0x20) != 0)) {
    piVar4 = FUN_00403040(param_2,&param_1);
    iVar5 = *piVar4;
    iVar1 = *(int *)(iVar2 + 0x28);
    FUN_00619730();
    if (iVar5 == iVar1) {
      iVar5 = 0;
    }
    else {
      iVar5 = *(int *)(*(int *)((int)this + 0xc) + 0x168);
      iVar6 = *(int *)((int)pvVar3 + 0xb4);
      if (*(int *)((int)pvVar3 + 0xb4) < iVar5) {
        iVar6 = iVar5;
      }
      iVar5 = *(int *)((int)pvVar3 + 0xa4);
    }
    if (((iVar6 <= *(int *)(iVar2 + 0x5c)) &&
        (*(int *)((int)pvVar3 + 0x118) <= *(int *)(iVar2 + 0x80))) &&
       ((((*(byte *)(iVar2 + 0x38) & 0x10) != 0 && (iVar5 <= *(int *)(iVar2 + 0x74))) ||
        ((0 < *(int *)(iVar2 + 0x9c) || (iVar5 <= *(int *)(iVar2 + 0x74))))))) {
      local_c = 1;
    }
  }
  return local_c;
}

