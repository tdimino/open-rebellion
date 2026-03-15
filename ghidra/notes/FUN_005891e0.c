
undefined4 __thiscall FUN_005891e0(void *this,void *param_1,uint *param_2,void *param_3)

{
  uint *puVar1;
  void *pvVar2;
  void *local_4;
  
  pvVar2 = param_3;
  puVar1 = param_2;
  *param_2 = 0;
  param_2 = (uint *)0x0;
  local_4 = (void *)0x0;
  FUN_00588650(*(void **)((int)this + 4),&param_2,&local_4,param_3);
  *puVar1 = (uint)(param_2 == (uint *)0x0);
  if ((param_2 == (uint *)0x0) == 0) {
    FUN_005888f0(*(void **)((int)this + 4),(int *)param_2,local_4,param_1,pvVar2);
  }
  return 1;
}

