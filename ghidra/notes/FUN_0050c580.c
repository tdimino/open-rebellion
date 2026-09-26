
int __thiscall FUN_0050c580(void *this,int *param_1,void *param_2)

{
  int *piVar1;
  uint *puVar2;
  int iVar3;
  void *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00641d28;
  local_c = ExceptionList;
  iVar3 = 1;
  ExceptionList = &local_c;
  local_10 = this;
  FUN_0050c640(param_1);
  local_4 = 0;
  puVar2 = FUN_004025b0(this,(uint *)&local_10);
  piVar1 = (int *)*puVar2;
  FUN_00619730();
  if (param_1 == piVar1) {
    if (((*(uint *)((int)this + 0x88) & 0x400) == 0) ||
       (((*(uint *)((int)this + 0x24) ^ param_1[9]) & 0xc0) != 0)) {
      iVar3 = 0;
    }
    else {
      iVar3 = 1;
    }
    iVar3 = FUN_00557860(param_1,iVar3,param_2);
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return iVar3;
}

