
void * __thiscall FUN_004a3130(void *this,void *param_1,int param_2,int param_3)

{
  bool bVar1;
  short sVar2;
  short sVar3;
  int *piVar4;
  int *piVar5;
  undefined4 *puVar6;
  int iVar7;
  undefined4 local_20;
  undefined4 local_1c;
  int iStack_18;
  undefined4 uStack_14;
  undefined4 local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00637a07;
  pvStack_c = ExceptionList;
  local_10 = 0;
  ExceptionList = &pvStack_c;
  FUN_004ece30(&local_20);
  local_4 = 1;
  sVar3 = (short)param_2;
  local_1c = CONCAT22((undefined2)param_3,sVar3);
  piVar5 = (int *)(**(code **)(**(int **)((int)this + 0x188) + 8))();
  if (piVar5 == (int *)0x0) {
    FUN_004f26d0(&local_20,(undefined4 *)((int)this + 0x144));
  }
  iStack_18 = 0;
  bVar1 = true;
  while (piVar4 = piVar5, piVar4 != (int *)0x0) {
    piVar5 = (int *)(**(code **)(*piVar4 + 0xc))();
    if ((bVar1) && ((*(byte *)(piVar4 + 0x1a) & 8) == 0)) {
      bVar1 = true;
    }
    else {
      bVar1 = false;
    }
    if (((*(byte *)(piVar4 + 0xf) & 1) != 0) && ((*(byte *)(piVar4 + 0x1a) & 8) != 0)) {
      if (iStack_18 == 0) {
        FUN_004f26d0(&local_20,piVar4 + 0x1b);
        iStack_18 = 1;
      }
      else {
        puVar6 = (undefined4 *)FUN_004ece30(&uStack_14);
        local_4._0_1_ = 2;
        FUN_004f26d0(&local_20,puVar6);
        local_4 = CONCAT31(local_4._1_3_,1);
        FUN_00619730();
      }
    }
  }
  if (bVar1) {
    FUN_004f26d0(&local_20,(undefined4 *)((int)this + 0x144));
  }
  sVar2 = (short)param_3;
  if ((((param_2 < 4) || (0x5f < param_2)) || (param_3 < 0x1d)) || (0x127 < param_3)) {
    if (((param_2 < 0x65) || (0xea < param_2)) ||
       ((param_3 < 0x7f || ((0x123 < param_3 || (*(int *)((int)this + 0x194) != 0x66))))))
    goto LAB_004a3300;
    local_1c = CONCAT22(sVar2 + -0x7f,sVar3 + -0x65);
    puVar6 = (undefined4 *)FUN_00609d80(*(void **)((int)this + 0x164),&param_3,local_1c);
    iVar7 = FUN_00609d10(*(void **)((int)this + 0x164),*puVar6);
  }
  else {
    FUN_004f26d0(&local_20,(undefined4 *)((int)this + 0x144));
    local_1c = CONCAT22(sVar2 + -0x1d,sVar3 + -4);
    puVar6 = (undefined4 *)FUN_00609d80(*(void **)((int)this + 0x160),&param_3,local_1c);
    iVar7 = FUN_00609d10(*(void **)((int)this + 0x160),*puVar6);
  }
  if (iVar7 != 0) {
    FUN_004f26d0(&local_20,(undefined4 *)(iVar7 + 0x6c));
  }
LAB_004a3300:
  FUN_004f26d0(param_1,&local_20);
  local_10 = 1;
  local_4 = local_4 & 0xffffff00;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return param_1;
}

