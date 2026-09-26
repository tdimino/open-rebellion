
int __thiscall FUN_0053c2e0(void *this,void *param_1,int *param_2)

{
  int *this_00;
  bool bVar1;
  int iVar2;
  int *this_01;
  int iVar3;
  undefined3 extraout_var;
  uint uVar4;
  void *pvVar5;
  uint *puVar6;
  void *unaff_retaddr;
  undefined4 local_14;
  void *local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  pvVar5 = param_1;
  puStack_8 = &LAB_00646428;
  pvStack_c = ExceptionList;
  local_14 = 1;
  local_10 = (void *)0xffffffff;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  FUN_00520580(param_1,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  this_00 = param_2;
  (**(code **)(*param_2 + 4))();
  iVar2 = (**(code **)(*(int *)this + 0x1c))(pvVar5);
  if ((iVar2 != 0) && (*(int *)((int)pvVar5 + 4) == -1)) {
    FUN_004f4340((undefined4 *)&stack0xffffffe8);
    puStack_8 = (undefined1 *)0x1;
    for (this_01 = (int *)thunk_FUN_005f5060(*(int *)((int)this + 0x44)); this_01 != (int *)0x0;
        this_01 = (int *)this_01[4]) {
      thunk_FUN_005f50a0(&stack0xffffffe8);
      iVar3 = (**(code **)(*this_01 + 0xc))();
      if ((iVar3 == 0x204) || (iVar3 = (**(code **)(*this_01 + 0xc))(), iVar3 == 0x241)) {
        bVar1 = true;
      }
      else {
        bVar1 = false;
      }
      if ((bVar1) && (iVar2 != 0)) {
        iVar2 = 1;
      }
      else {
        iVar2 = 0;
      }
      iVar3 = (**(code **)(*this_01 + 0xc))();
      if (iVar3 == 0x204) {
        bVar1 = FUN_0053d200(this_01,unaff_retaddr,(int *)&stack0xffffffe8);
        if ((CONCAT31(extraout_var,bVar1) == 0) || (iVar2 == 0)) {
LAB_0053c417:
          iVar2 = 0;
        }
        else {
          iVar2 = 1;
        }
      }
      else {
        iVar3 = (**(code **)(*this_01 + 0xc))();
        if (iVar3 == 0x241) {
          uVar4 = FUN_004f4850(this_01,unaff_retaddr,(int *)&stack0xffffffe8);
          if ((uVar4 == 0) || (iVar2 == 0)) goto LAB_0053c417;
          iVar2 = 1;
        }
      }
      if (*(int *)((int)unaff_retaddr + 4) == -1) {
        pvVar5 = (void *)thunk_FUN_005f5060((int)&stack0xffffffe8);
        while (pvVar5 != (void *)0x0) {
          puVar6 = FUN_00403040(pvVar5,&param_1);
          puStack_8._0_1_ = 2;
          FUN_004f5910(&stack0xffffffe8,puVar6);
          puStack_8 = (undefined1 *)CONCAT31(puStack_8._1_3_,1);
          FUN_00619730();
          FUN_004f57b0(this_00,pvVar5);
          pvVar5 = (void *)thunk_FUN_005f5060((int)&stack0xffffffe8);
        }
      }
    }
    puStack_8 = (undefined1 *)0xffffffff;
    FUN_004f4380((undefined4 *)&stack0xffffffe8);
  }
  ExceptionList = local_10;
  return iVar2;
}

