
bool __thiscall FUN_005530d0(void *this,uint *param_1,int *param_2,undefined4 *param_3)

{
  int iVar1;
  void *pvVar2;
  bool bVar3;
  bool bVar4;
  
  iVar1 = *(int *)((int)this + 0x10);
  *param_2 = 0;
  bVar3 = false;
  if (iVar1 == -1) {
    pvVar2 = FUN_004f5940(this,param_1);
    *param_2 = (int)pvVar2;
    bVar3 = pvVar2 != (void *)0x0;
  }
  *param_3 = 0;
  bVar4 = false;
  if (bVar3) {
    pvVar2 = FUN_004f5940(this,(uint *)(*param_2 + 0x20));
    *param_3 = pvVar2;
    bVar4 = pvVar2 != (void *)0x0;
  }
  return bVar4;
}

