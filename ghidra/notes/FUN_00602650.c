
void __thiscall FUN_00602650(void *this,char *param_1)

{
  void *pvVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0065629b;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  if (*(undefined4 **)((int)this + 200) != (undefined4 *)0x0) {
    ExceptionList = &local_c;
    (**(code **)**(undefined4 **)((int)this + 200))(1);
  }
  if (param_1 == (char *)0x0) {
    *(undefined4 *)((int)this + 200) = 0;
  }
  else {
    pvVar1 = (void *)FUN_00618b70(0xc);
    local_4 = 0;
    if (pvVar1 == (void *)0x0) {
      pvVar1 = (void *)0x0;
    }
    else {
      pvVar1 = FUN_005f35b0(pvVar1,param_1);
    }
    local_4 = 0xffffffff;
    *(void **)((int)this + 200) = pvVar1;
  }
  FUN_006026e0(this);
  ExceptionList = local_c;
  return;
}

