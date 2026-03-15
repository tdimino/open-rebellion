
uint __thiscall FUN_0052b8c0(void *this,undefined4 param_1,uint param_2,void *param_3)

{
  void *pvVar1;
  uint *puVar2;
  uint uVar3;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00644b28;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  pvVar1 = (void *)FUN_0052bed0((int)this);
  uVar3 = (uint)(pvVar1 != (void *)0x0);
  if ((uVar3 != 0) && (param_2 == 0)) {
    puVar2 = FUN_004025b0(pvVar1,&param_2);
    pvVar1 = param_3;
    local_4 = 0;
    uVar3 = FUN_005298d0(this,(int *)puVar2,param_3);
    local_4 = 0xffffffff;
    FUN_00619730();
    if (uVar3 != 0) {
      uVar3 = FUN_005294e0(this,0,pvVar1);
      if (uVar3 != 0) {
        uVar3 = FUN_00529540(this,0,pvVar1);
      }
    }
  }
  ExceptionList = local_c;
  return uVar3;
}

