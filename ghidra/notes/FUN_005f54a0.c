
void __thiscall FUN_005f54a0(void *this,uint param_1)

{
  undefined4 *puVar1;
  
  puVar1 = (undefined4 *)FUN_005f5500(this,param_1);
  if (puVar1 != (undefined4 *)0x0) {
    FUN_005f4fa0(this,(int)puVar1);
    (**(code **)*puVar1)(1);
  }
  return;
}

