
void * __thiscall FUN_00615ad0(void *this,char *param_1,int param_2,int param_3)

{
  void *pvVar1;
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00657125;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  if (param_3 != 0) {
    ExceptionList = &local_c;
    *(undefined **)this = &DAT_0066e518;
    FUN_006153b0((undefined4 *)((int)this + 0xc));
    local_4 = 0;
  }
  pvVar1 = (void *)FUN_00618b70(100);
  local_4 = 1;
  if (pvVar1 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_00615530(pvVar1,param_1,param_2,0);
  }
  local_4 = local_4 & 0xffffff00;
  FUN_006179e0(this,(int)pvVar1,0);
  *(undefined ***)(*(int *)(*(int *)this + 4) + (int)this) = &PTR_LAB_0066e514;
  *(undefined4 *)(*(int *)(*(int *)this + 4) + 0x1c + (int)this) = 1;
  ExceptionList = local_c;
  return this;
}

