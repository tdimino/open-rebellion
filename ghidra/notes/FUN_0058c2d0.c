
undefined4 __thiscall FUN_0058c2d0(void *this,void *param_1)

{
  void *this_00;
  bool bVar1;
  int iVar2;
  int iVar3;
  undefined4 uVar4;
  int local_18 [2];
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064fed0;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  iVar2 = FUN_0050a6d0(*(void **)((int)this + 0x10),1,param_1);
  this_00 = (void *)((int)this + 8);
  FUN_0058bc90(local_18,this_00);
  local_4 = 0;
  iVar3 = FUN_0058ba10(this_00,local_18,param_1);
  if ((iVar3 == 0) || (iVar2 == 0)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  *(undefined4 *)((int)this + 0x1c) = local_10;
  *(undefined4 *)((int)this + 0x24) = local_10;
  local_4 = 0xffffffff;
  FUN_0058bcd0(local_18);
  FUN_0058bc90(local_18,this_00);
  local_4 = 1;
  iVar2 = FUN_0058bbd0(this_00,local_18,param_1);
  if ((iVar2 == 0) || (!bVar1)) {
    uVar4 = 0;
  }
  else {
    uVar4 = 1;
  }
  *(undefined4 *)((int)this + 0x20) = local_10;
  *(undefined4 *)((int)this + 0x28) = local_10;
  local_4 = 0xffffffff;
  FUN_0058bcd0(local_18);
  ExceptionList = local_c;
  return uVar4;
}

