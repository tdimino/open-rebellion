
void __fastcall FUN_00609ae0(void *param_1)

{
  uint *puVar1;
  int *piVar2;
  void *pvVar3;
  int iVar4;
  int iVar5;
  int iVar6;
  uint uVar7;
  int iVar8;
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0065686b;
  pvStack_c = ExceptionList;
  iVar6 = 0;
  iVar8 = 0;
  iVar5 = 0;
  local_10 = 0;
  ExceptionList = &pvStack_c;
  piVar2 = (int *)(**(code **)(**(int **)((int)param_1 + 0xa0) + 8))();
  do {
    if (piVar2 == (int *)0x0) {
      if ((*(uint *)((int)param_1 + 0xf4) & 0x80000) != 0) {
        if ((local_10 != 0) && (iVar5 != 0)) {
          iVar8 = iVar8 + *(int *)((int)param_1 + 0xcc);
        }
        if ((iVar6 != 0) && (iVar8 != 0)) {
          iVar5 = iVar5 + *(int *)((int)param_1 + 0xd0);
        }
      }
      iVar6 = *(int *)((int)param_1 + 0x3c);
      if (iVar6 <= iVar5) {
        iVar6 = iVar5 + 5;
      }
      if (iVar8 < *(int *)((int)param_1 + 0x38)) {
        iVar8 = *(int *)((int)param_1 + 0x38);
      }
      iVar5 = FUN_005fc0e0(*(int **)((int)param_1 + 0xa4));
      if ((iVar5 < iVar8) || (iVar5 = FUN_005fc0f0(*(int **)((int)param_1 + 0xa4)), iVar5 < iVar6))
      {
        puVar1 = *(uint **)((int)param_1 + 0xa4);
        pvVar3 = (void *)FUN_00618b70(0x28);
        uStack_4 = 0;
        if (pvVar3 == (void *)0x0) {
          pvVar3 = (void *)0x0;
        }
        else {
          pvVar3 = FUN_005fbda0(pvVar3,iVar8,iVar6,puVar1);
        }
        uStack_4 = 0xffffffff;
        *(void **)((int)param_1 + 0xa4) = pvVar3;
        if (puVar1 != (uint *)0x0) {
          FUN_005fbfa0(puVar1);
          FUN_00618b60((undefined *)puVar1);
        }
      }
      if (*(int *)((int)param_1 + 0xf0) == 0) {
        uVar7 = iVar8 - *(int *)((int)param_1 + 0x38);
      }
      else {
        uVar7 = iVar6 - *(int *)((int)param_1 + 0x3c);
      }
      *(uint *)((int)param_1 + 200) = uVar7;
      uVar7 = uVar7 & ((int)uVar7 < 0) - 1;
      *(uint *)((int)param_1 + 200) = uVar7;
      if ((int)uVar7 < *(int *)((int)param_1 + 0xc4)) {
        *(uint *)((int)param_1 + 0xc4) = uVar7;
      }
      FUN_0060a670((void *)((int)param_1 + 0x94),uVar7 + *(int *)((int)param_1 + 0x3c));
      FUN_0060a730((void *)((int)param_1 + 0x94),
                   *(int *)((int)param_1 + 0x38) + *(int *)((int)param_1 + 200));
      FUN_00609e20(param_1);
      ExceptionList = pvStack_c;
      return;
    }
    piVar2[10] = iVar8;
    piVar2[0xb] = iVar5;
    if ((*(uint *)((int)param_1 + 0xf4) & 0x80000) == 0) {
      if (*(int *)((int)param_1 + 0xf0) == 0) goto LAB_00609b8a;
      iVar4 = *(int *)((int)param_1 + 0xd0);
LAB_00609d07:
      iVar5 = iVar5 + iVar4;
    }
    else {
      if (*(int *)((int)param_1 + 0x98) == 0) {
        if (*(int *)((int)param_1 + 0x9c) == 0) {
LAB_00609b67:
          iVar6 = *(int *)((int)param_1 + 0x38);
        }
        else {
          local_10 = *(int *)((int)param_1 + 0x3c) - *(int *)(*(int *)((int)param_1 + 0x9c) + 0x3c);
        }
      }
      else {
        if (*(int *)((int)param_1 + 0x9c) != 0) goto LAB_00609b67;
        iVar6 = *(int *)((int)param_1 + 0x38) - *(int *)(*(int *)((int)param_1 + 0x98) + 0x38);
      }
      if (local_10 == 0) {
        if (iVar6 != 0) {
          if (iVar6 < iVar8 + *(int *)((int)param_1 + 0xcc) * 2) {
            iVar8 = 0;
            iVar5 = iVar5 + *(int *)((int)param_1 + 0xd0);
          }
          else {
            iVar8 = iVar8 + *(int *)((int)param_1 + 0xcc);
          }
        }
      }
      else {
        iVar4 = *(int *)((int)param_1 + 0xd0);
        if (iVar5 + iVar4 * 2 <= *(int *)((int)param_1 + 0x3c)) goto LAB_00609d07;
        iVar5 = 0;
LAB_00609b8a:
        iVar8 = iVar8 + *(int *)((int)param_1 + 0xcc);
      }
    }
    piVar2 = (int *)(**(code **)(*piVar2 + 0xc))();
  } while( true );
}

