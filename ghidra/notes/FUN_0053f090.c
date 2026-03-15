
int * __cdecl FUN_0053f090(int param_1,uint *param_2,int param_3)

{
  bool bVar1;
  undefined3 extraout_var;
  void *this;
  undefined3 extraout_var_00;
  int iVar2;
  int *piVar3;
  
  piVar3 = (int *)0x0;
  if ((param_1 != 0) && (bVar1 = FUN_00539ff0(param_1), CONCAT31(extraout_var,bVar1) != 0)) {
    this = *(void **)(param_1 + 0x20);
    while (this = (void *)FUN_00584570(this,param_2), this != (void *)0x0) {
      bVar1 = FUN_00584510((int)this);
      if (((CONCAT31(extraout_var_00,bVar1) != 0) &&
          (piVar3 = (int *)FUN_005844e0(this,*(uint *)(param_1 + 0x24) >> 4 & 3), param_3 != 0)) &&
         (iVar2 = (**(code **)(*piVar3 + 0x10))(), iVar2 != 0)) {
        piVar3 = (int *)0x0;
      }
      if (this == (void *)0x0) {
        return piVar3;
      }
      if (piVar3 != (int *)0x0) {
        return piVar3;
      }
    }
  }
  return piVar3;
}

