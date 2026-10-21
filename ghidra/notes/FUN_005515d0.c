
int __thiscall FUN_005515d0(void *this,uint *param_1)

{
  uint *puVar1;
  bool bVar2;
  void *pvVar3;
  undefined3 extraout_var;
  int iVar4;
  void *local_4;
  
  puVar1 = param_1;
  iVar4 = 1;
  local_4 = this;
  pvVar3 = FUN_004f5940(this,param_1);
  if (pvVar3 != (void *)0x0) {
    param_1 = (uint *)0x0;
    local_4 = (void *)0x0;
    bVar2 = FUN_005530d0(this,puVar1,(int *)&param_1,&local_4);
    iVar4 = 0;
    if (CONCAT31(extraout_var,bVar2) != 0) {
      iVar4 = FUN_00551050((int)param_1);
    }
  }
  return iVar4;
}

