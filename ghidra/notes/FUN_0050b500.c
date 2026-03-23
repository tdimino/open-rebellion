
void __thiscall FUN_0050b500(void *this,void *param_1)

{
  uint uVar1;
  int iVar2;
  void *pvVar3;
  int iVar4;
  undefined4 local_2c [8];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00641b68;
  pvStack_c = ExceptionList;
  iVar4 = 0;
  uVar1 = *(uint *)((int)this + 0x24) >> 6 & 3;
  if (uVar1 == 1) {
    iVar2 = 1;
  }
  else if (uVar1 == 2) {
    iVar2 = 2;
  }
  else {
    iVar2 = 0;
  }
  ExceptionList = &pvStack_c;
  if (iVar2 != 0) {
    ExceptionList = &pvStack_c;
    pvVar3 = FUN_00504c40(local_2c,this,1);
    local_4 = 0;
    iVar4 = FUN_00513180((int)pvVar3);
    local_4 = 0xffffffff;
    FUN_00504d40(local_2c);
    iVar4 = iVar4 - *(int *)((int)this + 0x80);
  }
  FUN_0050a230(this,iVar4,param_1);
  ExceptionList = pvStack_c;
  return;
}

