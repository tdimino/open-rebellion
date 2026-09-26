
int __thiscall FUN_0052fbf0(void *this,undefined4 *param_1,void *param_2)

{
  void *pvVar1;
  undefined4 *this_00;
  uint *puVar2;
  void *this_01;
  void *pvVar3;
  undefined4 uVar4;
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  pvVar1 = param_2;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00645398;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_1 = 0;
  local_10 = 1;
  if ((0 < *(int *)((int)this + 0x80)) && (0 < *(int *)((int)this + 0x78))) {
    this_00 = (undefined4 *)FUN_005335c0(this,param_2);
    puVar2 = FUN_0052fd10(this_00,&param_2);
    local_4 = 0;
    this_01 = (void *)FUN_00505b20(puVar2);
    local_4 = 0xffffffff;
    FUN_00619730();
    if ((this_01 != (void *)0x0) && (*(int *)((int)this_01 + 0x58) == 1)) {
      param_2 = (void *)this_00[5];
      pvVar3 = (void *)FUN_0053a6e0((int)this_01);
      if (param_2 == pvVar3) {
        param_2 = (void *)0x0;
        local_10 = FUN_0052fb30(this,(uint *)&param_2,pvVar1);
        if ((local_10 != 0) && (param_2 != (void *)0x0)) {
          local_10 = FUN_0053a860(this_01,1,pvVar1);
        }
      }
    }
    if (this_00 != (undefined4 *)0x0) {
      (**(code **)*this_00)(1);
    }
  }
  if (local_10 != 0) {
    if ((*(int *)((int)this + 0x80) < 1) || (*(int *)((int)this + 0x78) < 1)) {
      uVar4 = 0;
    }
    else {
      uVar4 = 1;
    }
    *param_1 = uVar4;
  }
  ExceptionList = local_c;
  return local_10;
}

