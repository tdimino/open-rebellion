
void __thiscall FUN_006025a0(void *this,uint param_1)

{
  void *pvVar1;
  CHAR local_10c [256];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0065627e;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_00601060(param_1 & 0xffff,local_10c,0x100);
  if (*(undefined4 **)((int)this + 200) != (undefined4 *)0x0) {
    (**(code **)**(undefined4 **)((int)this + 200))(1);
  }
  pvVar1 = (void *)FUN_00618b70(0xc);
  local_4 = 0;
  if (pvVar1 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_005f35b0(pvVar1,local_10c);
  }
  local_4 = 0xffffffff;
  *(void **)((int)this + 200) = pvVar1;
  FUN_006026e0(this);
  ExceptionList = local_c;
  return;
}

