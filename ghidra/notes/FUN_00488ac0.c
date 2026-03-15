
void __thiscall FUN_00488ac0(void *this,void *param_1)

{
  FUN_005f4db0(param_1,this);
  FUN_005f4e30(param_1,(void *)((int)this + 8));
  (**(code **)(*(int *)((int)this + 0x10) + 0x10))(param_1);
  (**(code **)(*(int *)((int)this + 0x1c) + 0x10))(param_1);
  FUN_005f5da0((void *)((int)this + 0x44),param_1);
  FUN_005f49a0((void *)((int)this + 0xc),param_1);
  FUN_005f4db0(param_1,(int)this + 0x40);
  FUN_005f4db0(param_1,(int)this + 0x50);
  (**(code **)(*(int *)((int)this + 0x34) + 0x10))(param_1);
  return;
}

