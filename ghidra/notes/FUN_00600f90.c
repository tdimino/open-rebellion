
bool __thiscall FUN_00600f90(void *this,uint param_1)

{
  undefined4 *puVar1;
  
  puVar1 = (undefined4 *)FUN_00604500((void *)((int)this + 0x6c),param_1);
  if (puVar1 != (undefined4 *)0x0) {
    FUN_005f4fa0((void *)((int)this + 0x6c),(int)puVar1);
    FUN_00600280((int)puVar1);
    (**(code **)*puVar1)(1);
  }
  return puVar1 != (undefined4 *)0x0;
}

