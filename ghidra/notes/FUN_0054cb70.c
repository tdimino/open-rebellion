
undefined4 __fastcall FUN_0054cb70(int param_1)

{
  bool bVar1;
  int iVar2;
  int iVar3;
  void *this;
  uint *puVar4;
  int *piVar5;
  undefined4 *puVar6;
  void *this_00;
  int local_18;
  uint local_14;
  int *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064811b;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  iVar2 = FUN_0054cb20(param_1);
  if (iVar2 != 0) {
    for (iVar3 = FUN_0051cad0(0x40); iVar3 != 0; iVar3 = *(int *)(iVar3 + 0x10)) {
      *(int *)(param_1 + 0x6c) = *(int *)(param_1 + 0x6c) + 1;
    }
    if ((iVar2 != 0) && (iVar2 = *(int *)(param_1 + 0x6c), iVar2 != 0)) {
      local_10 = (int *)FUN_00618b70(iVar2 * 0xc + 4);
      local_4 = 0;
      if (local_10 == (int *)0x0) {
        piVar5 = (int *)0x0;
      }
      else {
        piVar5 = local_10 + 1;
        *local_10 = iVar2;
        FUN_00618c80(piVar5,0xc,iVar2,FUN_0054cd20);
      }
      local_4 = 0xffffffff;
      *(int **)(param_1 + 0x70) = piVar5;
      if (piVar5 != (int *)0x0) {
        this_00 = *(void **)(param_1 + 0x70);
        this = (void *)FUN_0051cad0(0x40);
        if (this == (void *)0x0) {
          ExceptionList = local_c;
          return 1;
        }
        local_18 = (int)this_00 + 4;
        bVar1 = true;
        do {
          puVar4 = (uint *)FUN_00402e40(this,&local_14);
          local_4 = 1;
          piVar5 = FUN_0053f100(puVar4);
          local_4 = 0xffffffff;
          FUN_00619730();
          if ((piVar5 == (int *)0x0) || (!bVar1)) {
            bVar1 = false;
          }
          else {
            bVar1 = true;
          }
          if (piVar5 != (int *)0x0) {
            puVar6 = (undefined4 *)FUN_00402e40(this,(uint *)&local_10);
            local_4 = 2;
            FUN_004f26d0(this_00,puVar6);
            local_4 = 0xffffffff;
            FUN_00619730();
            iVar2 = (**(code **)(*piVar5 + 0x1bc))(local_18);
            if ((iVar2 == 0) || (!bVar1)) {
              bVar1 = false;
            }
            else {
              bVar1 = true;
            }
            if (piVar5 != (int *)0x0) {
              (**(code **)*piVar5)(1);
            }
          }
          this = *(void **)((int)this + 0x10);
          this_00 = (void *)((int)this_00 + 0xc);
          local_18 = local_18 + 0xc;
        } while (this != (void *)0x0);
        ExceptionList = local_c;
        return 1;
      }
      *(undefined4 *)(param_1 + 0x6c) = 0;
    }
  }
  FUN_0054cb20(param_1);
  ExceptionList = local_c;
  return 1;
}

