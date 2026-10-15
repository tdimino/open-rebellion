
int * __cdecl FUN_0053f030(uint *param_1,int param_2,int param_3)

{
  bool bVar1;
  void *this;
  undefined3 extraout_var;
  int iVar2;
  int *piVar3;
  
  piVar3 = (int *)0x0;
  if (DAT_006b90e4 != (void *)0x0) {
    this = FUN_00585e40(DAT_006b90e4,param_1);
    if (this != (void *)0x0) {
      bVar1 = FUN_00584510((int)this);
      if (CONCAT31(extraout_var,bVar1) != 0) {
        piVar3 = (int *)FUN_005844e0(this,param_2);
        if (param_3 != 0) {
          iVar2 = (**(code **)(*piVar3 + 0x10))();
          if (iVar2 != 0) {
            piVar3 = FUN_0053f090((int)piVar3,param_1,1);
          }
        }
      }
    }
  }
  return piVar3;
}

