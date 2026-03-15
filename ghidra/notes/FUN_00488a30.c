
void __thiscall FUN_00488a30(void *this,void *param_1)

{
  FUN_005f4d90(param_1,this);
  FUN_005f4e10(param_1,(void *)((int)this + 8));
  (**(code **)(*(int *)((int)this + 0x10) + 0xc))(param_1);
  (**(code **)(*(int *)((int)this + 0x1c) + 0xc))(param_1);
  (**(code **)(*(int *)((int)this + 0x44) + 4))();
  FUN_005f5cf0((void *)((int)this + 0x44),param_1);
  FUN_005f4990((void *)((int)this + 0xc),param_1);
  FUN_005f4d90(param_1,(int)this + 0x40);
  FUN_005f4d90(param_1,(uint *)((int)this + 0x50));
  FUN_0041db20(*(uint *)((int)this + 0x50));
  (**(code **)(*(int *)((int)this + 0x34) + 0xc))(param_1);
  return;
}

