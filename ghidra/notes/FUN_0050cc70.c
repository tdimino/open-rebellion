
int __thiscall FUN_0050cc70(void *this,void *param_1)

{
  bool bVar1;
  bool bVar2;
  void *pvVar3;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  int iVar4;
  int iVar5;
  int local_34;
  int local_30;
  undefined4 local_2c [8];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00641de0;
  pvStack_c = ExceptionList;
  local_30 = *(int *)((int)this + 0x5c);
  local_34 = *(int *)((int)this + 100);
  ExceptionList = &pvStack_c;
  pvVar3 = FUN_0052d690(local_2c,this,1,*(uint *)((int)this + 0x24) >> 6 & 3);
  local_4 = 0;
  bVar1 = FUN_005131b0((int)pvVar3);
  local_4 = 0xffffffff;
  FUN_0052d710(local_2c);
  pvVar3 = FUN_00539df0(local_2c,this,1,*(uint *)((int)this + 0x24) >> 6 & 3);
  local_4 = 1;
  bVar2 = FUN_005131b0((int)pvVar3);
  local_4 = 0xffffffff;
  FUN_00539e70(local_2c);
  iVar4 = FUN_00559ee0(&local_34,&local_30,CONCAT31(extraout_var,bVar1),
                       CONCAT31(extraout_var_00,bVar2));
  if ((iVar4 != 0) &&
     ((*(int *)((int)this + 100) != local_34 || (*(int *)((int)this + 0x5c) != local_30)))) {
    FUN_0050aba0(this,1,param_1);
    iVar4 = FUN_00509dc0(this,local_34,param_1);
    iVar5 = FUN_00509cc0(this,local_30,param_1);
    if ((iVar5 == 0) || (iVar4 == 0)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    iVar4 = FUN_0050aba0(this,0,param_1);
    if ((iVar4 != 0) && (bVar1)) {
      ExceptionList = pvStack_c;
      return 1;
    }
    iVar4 = 0;
  }
  ExceptionList = pvStack_c;
  return iVar4;
}

