
int __thiscall FUN_00504990(void *this,int param_1,void *param_2)

{
  bool bVar1;
  int iVar2;
  undefined3 extraout_var;
  
  iVar2 = FUN_004f2b90(this,param_1,param_2);
  if ((iVar2 != 0) && (param_1 != 0)) {
    bVar1 = FUN_0053e2f0(*(int *)((int)this + 0x60));
    if (CONCAT31(extraout_var,bVar1) == 0) {
      iVar2 = FUN_00504400(this,1,param_2);
    }
  }
  return iVar2;
}

