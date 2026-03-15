
void __thiscall FUN_00476e80(void *this,undefined4 param_1,undefined4 *param_2,undefined4 *param_3)

{
  switch(param_1) {
  case 0x12:
    (**(code **)(**(int **)((int)this + 0x5c) + 0x30))(param_2,param_3);
    return;
  case 0x13:
    (**(code **)(**(int **)((int)this + 0x5c) + 0x28))(param_2,param_3);
    return;
  default:
    FUN_004ece80(param_2);
    FUN_004ece80(param_3);
    return;
  case 0x15:
    (**(code **)(**(int **)((int)this + 0x5c) + 0x2c))(param_2,param_3);
    return;
  case 0x17:
  case 0x81:
    (**(code **)(**(int **)((int)this + 0x5c) + 0x24))(param_2,param_3);
    return;
  }
}

