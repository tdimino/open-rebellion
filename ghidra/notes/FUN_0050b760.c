
int __thiscall FUN_0050b760(void *this,void *param_1)

{
  bool bVar1;
  undefined3 extraout_var;
  int iVar2;
  undefined4 local_2c [8];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00641bb8;
  local_c = ExceptionList;
  iVar2 = 1;
  ExceptionList = &local_c;
  if (((*(byte *)((int)this + 0x88) & 1) == 0) &&
     (ExceptionList = &local_c, (*(byte *)((int)this + 0x50) & 8) == 0)) {
    ExceptionList = &local_c;
    FUN_00539d70(local_2c,this,1);
    local_4 = 0;
    bVar1 = FUN_005131b0((int)local_2c);
    iVar2 = FUN_0050a3c0(this,CONCAT31(extraout_var,bVar1),param_1);
    local_4 = 0xffffffff;
    FUN_00539e70(local_2c);
  }
  if ((iVar2 != 0) && ((*(byte *)((int)this + 0x50) & 8) != 0)) {
    iVar2 = FUN_0050a3c0(this,0,param_1);
  }
  ExceptionList = local_c;
  return iVar2;
}

