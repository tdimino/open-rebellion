
uint __thiscall
FUN_00529d20(void *this,int *param_1,undefined4 param_2,undefined4 param_3,void *param_4)

{
  int iVar1;
  int iVar2;
  int *piVar3;
  uint uVar4;
  void *local_4;
  
  uVar4 = 1;
  local_4 = this;
  piVar3 = (int *)FUN_0040d760(this,&local_4);
  iVar1 = *piVar3;
  iVar2 = *param_1;
  FUN_00619730();
  if (iVar2 == iVar1) {
    uVar4 = FUN_0052ac40(this,param_4);
  }
  return uVar4;
}

