
void * __thiscall FUN_00402d80(void *this,void *param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  undefined4 local_14;
  uint local_10;
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006293d5;
  local_c = ExceptionList;
  if (*(void **)((int)this + 0x2c) == (void *)0x0) {
    ExceptionList = &local_c;
    puVar1 = (undefined4 *)FUN_0042dbe0(&local_14);
    uVar2 = 2;
  }
  else {
    ExceptionList = &local_c;
    puVar1 = (undefined4 *)FUN_00402e40(*(void **)((int)this + 0x2c),&local_10);
    uVar2 = 1;
  }
  local_4 = uVar2;
  FUN_004f26d0(param_1,puVar1);
  local_4 = 1;
  if ((uVar2 & 2) != 0) {
    uVar2 = uVar2 & 0xfffffffd;
    FUN_00619730();
  }
  local_4 = 0;
  if ((uVar2 & 1) != 0) {
    FUN_00619730();
  }
  ExceptionList = local_c;
  return param_1;
}

