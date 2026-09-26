
float10 __thiscall FUN_005cb650(void *param_1,int *param_2)

{
  int *piVar1;
  int iVar2;
  void *pvVar3;
  float10 fVar4;
  int *piVar5;
  float local_4;
  
  local_4 = 0.0;
  for (piVar1 = (int *)(**(code **)(*(int *)((int)param_1 + 0x1c) + 0x10))(); piVar1 != (int *)0x0;
      piVar1 = (int *)(**(code **)(*piVar1 + 8))()) {
    iVar2 = (**(code **)(*piVar1 + 0x10))();
    if (((*(int *)(iVar2 + 0x170) == 2) ||
        (iVar2 = (**(code **)(*piVar1 + 0x10))(), *(int *)(iVar2 + 0x170) == 6)) ||
       (iVar2 = (**(code **)(*piVar1 + 0x10))(), *(int *)(iVar2 + 0x170) == 4)) {
      piVar5 = param_2;
      pvVar3 = (void *)(**(code **)(*piVar1 + 0x10))(param_2,0);
      fVar4 = FUN_005b7780(pvVar3,piVar5);
      local_4 = (float)(fVar4 + (float10)local_4);
    }
  }
  return (float10)local_4;
}

