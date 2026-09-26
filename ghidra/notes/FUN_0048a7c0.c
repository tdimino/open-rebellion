
void __thiscall FUN_0048a7c0(void *this,void *param_1)

{
  undefined4 *puVar1;
  int *piVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00634f5f;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f4d90(param_1,(int)this + 0x24);
  FUN_005f4e10(param_1,(void *)((int)this + 0x14));
  FUN_005f4d90(param_1,(int *)((int)this + 0x18));
  FUN_005f4d90(param_1,(int)this + 0x1c);
  if (*(int *)((int)this + 0x18) != 0) {
    puVar1 = (undefined4 *)FUN_00618b70(0x4c);
    if (puVar1 == (undefined4 *)0x0) {
      FUN_00619730();
    }
    local_4 = 0;
    if (puVar1 == (undefined4 *)0x0) {
      piVar2 = (int *)0x0;
    }
    else {
      piVar2 = FUN_00440910(puVar1);
    }
    *(int **)((int)this + 0x20) = piVar2;
    local_4 = 0xffffffff;
    (**(code **)(*piVar2 + 4))(param_1);
  }
  FUN_005f4d90(param_1,(int)this + 0x28);
  FUN_005f4d90(param_1,(int)this + 0x2c);
  FUN_00536f00((void *)((int)this + 4),param_1);
  ExceptionList = local_c;
  return;
}

