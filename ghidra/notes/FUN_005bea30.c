
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined4 __thiscall FUN_005bea30(void *this,float param_1)

{
  uint uVar1;
  float fVar2;
  float fVar3;
  DWORD DVar4;
  void *pvVar5;
  int *piVar6;
  int iVar7;
  uint uVar8;
  uint uVar9;
  longlong lVar10;
  undefined1 uStack_20;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0065377b;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  DVar4 = timeGetTime();
  if (*(int *)((int)this + 0x8c0) != 0) {
    uVar8 = *(uint *)((int)this + 0x8ac);
    *(undefined4 *)((int)this + 0x8c0) = 0;
    *(DWORD *)((int)this + 0x89c) = DVar4;
    *(DWORD *)((int)this + 0x8bc) = DVar4;
    *(float *)((int)this + 0x9e8) = (float)uVar8;
    *(undefined4 *)((int)this + 0x8a8) = 1000;
    *(undefined4 *)((int)this + 0x8b0) = 0;
    *(undefined4 *)((int)this + 0x8b4) = 4;
    *(undefined4 *)((int)this + 0x8b8) = 0;
    *(undefined4 *)((int)this + 0x878) = 0;
    *(uint *)((int)this + 0x87c) = uVar8;
    uStack_20 = 0x47;
    if (*(int *)((int)this + 0x9ec) == 0) {
      uStack_20 = 0x49;
    }
    uVar9 = 0;
    if (uVar8 != 0) {
      do {
        pvVar5 = (void *)FUN_00618b70(0x220);
        uStack_4 = 0;
        if (pvVar5 == (void *)0x0) {
          pvVar5 = (void *)0x0;
        }
        else {
          pvVar5 = FUN_005ce500(pvVar5,(float)uVar9,uStack_20,1,1,(uint *)0x0);
        }
        uStack_4 = 0xffffffff;
        (**(code **)(*(int *)this + 0x2c))(pvVar5,0);
        uVar9 = uVar9 + 1;
      } while (uVar9 < *(uint *)((int)this + 0x8ac));
    }
  }
  uVar9 = DVar4 - *(int *)((int)this + 0x89c);
  uVar8 = *(uint *)((int)this + 0x8a8);
  if (uVar8 <= uVar9) {
    uVar9 = uVar9 % uVar8 + uVar8;
  }
  pvVar5 = (void *)FUN_0059ed70();
  FUN_005a0820(pvVar5);
  fVar3 = _DAT_0066c770;
  uVar8 = *(uint *)((int)this + 0x8a8);
  if (uVar8 <= uVar9) {
    if (*(int *)((int)this + 0x8b0) == 5) {
      if (*(int *)((int)this + 0xa08) == 1) {
        *(undefined4 *)((int)this + 0x8c4) = 1;
        ExceptionList = pvStack_c;
        return 1;
      }
      if ((*(int *)((int)this + 0x8b4) == 5) &&
         (fVar2 = *(float *)((int)this + 0xe70) - (float)uVar8,
         *(float *)((int)this + 0xe70) = fVar2, fVar2 <= fVar3)) {
        *(undefined4 *)((int)this + 0x8b4) = 6;
      }
      if (*(int *)((int)this + 0x8b4) == 6) {
        for (piVar6 = (int *)(**(code **)(*(int *)((int)this + 0x9a0) + 0x10))();
            piVar6 != (int *)0x0; piVar6 = (int *)(**(code **)(*piVar6 + 8))()) {
          iVar7 = (**(code **)(*piVar6 + 0x10))();
          *(undefined4 *)(iVar7 + 0x108) = 1;
          iVar7 = (**(code **)(*piVar6 + 0x10))();
          FUN_0059ed70();
          FUN_0059f4d0(iVar7);
        }
        iVar7 = FUN_0059ed70();
        FUN_005a6150(iVar7);
        ExceptionList = pvStack_c;
        return 1;
      }
      *(undefined4 *)((int)this + 0x8b0) = 0;
      uVar9 = uVar9 - uVar8;
      *(int *)((int)this + 0x878) = *(int *)((int)this + 0x878) + 1;
    }
    else {
      param_1 = 0.0;
    }
  }
  iVar7 = *(int *)((int)this + 0x8b0);
  if (iVar7 == 0) {
    uVar1 = *(uint *)((int)this + 0x878);
    *(DWORD *)((int)this + 0x89c) = DVar4 - uVar9;
    *(float *)((int)this + 0x9e8) = (float)(*(uint *)((int)this + 0x8ac) + uVar1);
    if (*(uint *)((int)this + 0x8ac) <= uVar1) {
      FUN_005a96b0(this,(float)(uVar1 - 1),(float)uVar8,0,(void *)((int)this + 0x908));
      FUN_005a96b0(this,(float)(*(int *)((int)this + 0x878) - 1),(float)*(uint *)((int)this + 0x8a8)
                   ,1,(void *)((int)this + 0x914));
    }
    *(undefined4 *)((int)this + 0x8b0) = 1;
  }
  else if (iVar7 == 1) {
    if (*(int *)((int)this + 0x8b4) == 4) {
      (**(code **)(*(int *)this + 0x80))((float)uVar8,*(undefined4 *)((int)this + 0x9ec));
      (**(code **)(*(int *)this + 0x80))
                ((float)*(uint *)((int)this + 0x8a8),*(int *)((int)this + 0x9ec) == 0);
      (**(code **)(*(int *)this + 0x84))((float)*(uint *)((int)this + 0x8a8));
      (**(code **)(*(int *)this + 0x88))((float)*(uint *)((int)this + 0x8a8));
    }
    (**(code **)(*(int *)this + 0x6c))
              ((float)*(uint *)((int)this + 0x878),*(undefined4 *)((int)this + 0x9ec));
    *(int *)((int)this + 0x87c) = *(int *)((int)this + 0x87c) + 1;
    iVar7 = (**(code **)(*(int *)this + 0x70))
                      ((float)*(uint *)((int)this + 0x878),*(int *)((int)this + 0x9ec) == 0,
                       *(undefined4 *)((int)this + 0x8b8));
    if (iVar7 == 1) {
      (**(code **)(*(int *)this + 0x94))(*(undefined4 *)((int)this + 0x878));
      *(undefined4 *)((int)this + 0x8b0) = 4;
    }
    else {
      *(undefined4 *)((int)this + 0x8b0) = 2;
    }
  }
  else if ((iVar7 == 2) &&
          (iVar7 = (**(code **)(*(int *)this + 0x70))
                             ((float)*(uint *)((int)this + 0x878),*(int *)((int)this + 0x9ec) == 0,
                              *(undefined4 *)((int)this + 0x8b8)), iVar7 == 1)) {
    (**(code **)(*(int *)this + 0x94))(*(undefined4 *)((int)this + 0x878));
    *(undefined4 *)((int)this + 0x8b0) = 4;
  }
  if ((*(int *)((int)this + 0x8b0) == 4) &&
     (iVar7 = (**(code **)(*(int *)this + 0x90))(*(undefined4 *)((int)this + 0x878)), iVar7 != 0)) {
    *(undefined4 *)((int)this + 0x8b0) = 5;
  }
  if (param_1 != _DAT_0066c770) {
    DVar4 = timeGetTime();
    param_1 = (float)((DVar4 - *(int *)((int)this + 0x8bc)) % *(uint *)((int)this + 0x8a8));
  }
  if (((param_1 != _DAT_0066c770) && (*(uint *)((int)this + 0x8ac) <= *(uint *)((int)this + 0x878)))
     && (*(int *)((int)this + 0x8b4) == 4)) {
    lVar10 = __ftol();
    uVar8 = *(int *)((int)this + 0x8bc) + (int)lVar10;
    *(uint *)((int)this + 0x8bc) = uVar8;
    (**(code **)(*(int *)this + 0x98))((float)uVar8 - param_1,param_1);
  }
  ExceptionList = pvStack_c;
  return 0;
}

