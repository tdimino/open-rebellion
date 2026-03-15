
undefined4 __thiscall FUN_0060dbb0(void *this,uint *param_1)

{
  uint uVar1;
  void *this_00;
  int iVar2;
  int iVar3;
  uint *puVar4;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00656c2b;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  uVar1 = FUN_0060dba0((int)this);
  if (uVar1 != 0) {
    ExceptionList = local_c;
    return 0;
  }
  FUN_0060dc80((undefined4 *)this);
  this_00 = (void *)FUN_00618b70(0x28);
  local_4 = 0;
  if (this_00 == (void *)0x0) {
    puVar4 = (uint *)0x0;
  }
  else {
    puVar4 = param_1;
    iVar2 = FUN_005fc0f0((int *)param_1);
    iVar3 = FUN_005fc0e0((int *)param_1);
    puVar4 = (uint *)FUN_005fbda0(this_00,iVar3,iVar2,puVar4);
  }
  local_4 = 0xffffffff;
  if (puVar4 == (uint *)0x0) {
    ExceptionList = local_c;
    return 0;
  }
  FUN_005fcc30(param_1,puVar4,0,0,0,0,(undefined4 *)0x0,0);
  *(uint **)this = puVar4;
  ExceptionList = local_c;
  return 1;
}

