
void FUN_00556390(uint *param_1,uint *param_2,void *param_3)

{
  uint *puVar1;
  void *pvVar2;
  int *this;
  int iVar3;
  uint uVar4;
  
  this = (int *)FUN_00504dc0(param_1);
  if ((this == (int *)0x0) || (iVar3 = (**(code **)(*this + 0x38))(), puVar1 = param_2, iVar3 == 0))
  {
    (**(code **)(*this + 0xa8))(param_2,param_3);
    return;
  }
  uVar4 = FUN_00555410(param_1,param_2,(uint *)&param_2);
  pvVar2 = param_3;
  if ((uVar4 != 0) && ((this[0x14] & 0x800U) == 0)) {
    uVar4 = FUN_004f7640(this,(uint)(param_2 == (uint *)0x0),param_3);
  }
  if (uVar4 == 0) {
    return;
  }
  if ((*(byte *)(this + 0x14) & 8) != 0) {
    return;
  }
  (**(code **)(*this + 0xa8))(puVar1,pvVar2);
  return;
}

