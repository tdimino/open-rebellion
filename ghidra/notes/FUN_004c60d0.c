
void __thiscall FUN_004c60d0(void *this,void *param_1,void *param_2)

{
  ushort uVar1;
  int iVar2;
  int iVar3;
  void *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063aa88;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  local_10 = this;
  FUN_004ece30(&local_10);
  uVar1 = 0;
  local_4 = 0;
  FUN_0058f0c0(this,param_1);
  FUN_005f4d90(param_1,(undefined4 *)((int)this + 0x1c));
  FUN_005f4d90(param_1,(int)this + 0x20);
  FUN_005f4990((void *)((int)this + 0x24),param_1);
  FUN_005f4990((void *)((int)this + 0x28),param_1);
  FUN_005f4dd0(param_1,(int)this + 0x2c);
  FUN_005f4dd0(param_1,(int)this + 0x2e);
  FUN_005f4dd0(param_1,(int)this + 0x30);
  do {
    FUN_004ece90(&local_10,param_1);
    FUN_004c67f0(this,uVar1,&local_10,param_2);
    uVar1 = uVar1 + 1;
  } while (uVar1 < 10);
  iVar3 = (int)this + 0x60;
  iVar2 = 0x1a;
  do {
    FUN_005f4d90(param_1,iVar3);
    iVar3 = iVar3 + 4;
    iVar2 = iVar2 + -1;
  } while (iVar2 != 0);
  local_4 = 0xffffffff;
  *(undefined4 *)((int)this + 0x1c) = 1;
  FUN_00619730();
  ExceptionList = local_c;
  return;
}

