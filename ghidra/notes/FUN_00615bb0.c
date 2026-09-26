
void * __thiscall FUN_00615bb0(void *this,int param_1)

{
  undefined4 *puVar1;
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00657155;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  if (param_1 != 0) {
    ExceptionList = &local_c;
    *(undefined **)this = &DAT_0066e528;
    FUN_006153b0((undefined4 *)((int)this + 8));
    local_4 = 0;
  }
  puVar1 = (undefined4 *)FUN_00618b70(100);
  local_4 = 1;
  if (puVar1 == (undefined4 *)0x0) {
    puVar1 = (undefined4 *)0x0;
  }
  else {
    puVar1 = FUN_006154e0(puVar1);
  }
  local_4 = local_4 & 0xffffff00;
  FUN_00616210(this,(int)puVar1,0);
  *(undefined ***)(*(int *)(*(int *)this + 4) + (int)this) = &PTR_LAB_0066e524;
  *(undefined4 *)(*(int *)(*(int *)this + 4) + 0x1c + (int)this) = 1;
  ExceptionList = local_c;
  return this;
}

