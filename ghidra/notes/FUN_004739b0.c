
void __thiscall FUN_004739b0(void *this,void *param_1)

{
  bool bVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  undefined4 uVar5;
  int iVar6;
  int iVar7;
  uint uVar8;
  int iVar9;
  int local_10;
  
  *(undefined4 *)((int)this + 0x30) = 0;
  *(undefined4 *)((int)this + 0x34) = 0;
  *(undefined4 *)((int)this + 0x3c) = 0;
  *(undefined4 *)((int)this + 0x40) = *(undefined4 *)((int)param_1 + 0xc0);
  *(undefined4 *)((int)this + 0x44) = *(undefined4 *)((int)param_1 + 0xe4);
  *(undefined4 *)((int)this + 0x48) = *(undefined4 *)((int)param_1 + 0xe8);
  *(undefined4 *)((int)this + 0x50) = *(undefined4 *)((int)param_1 + 0xdc);
  *(undefined4 *)((int)this + 0x54) = *(undefined4 *)((int)param_1 + 0xe0);
  local_10 = 0;
  if ((*(uint *)((int)param_1 + 0x2c) < 0x18) || (0x1b < *(uint *)((int)param_1 + 0x2c))) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  if (bVar1) {
    *(uint *)((int)this + 0x18) = *(uint *)((int)this + 0x18) | 0x80;
    goto LAB_00473ee2;
  }
  if (0 < *(int *)((int)param_1 + 0xcc)) {
    *(uint *)((int)this + 0x18) = *(uint *)((int)this + 0x18) | 0x400;
  }
  *(uint *)((int)this + 0x18) = *(uint *)((int)this + 0x18) | 0x100000;
  *(int *)((int)this + 0x3c) = *(int *)((int)param_1 + 0xe0) * 100;
  iVar6 = FUN_0053bb80(param_1,0,0);
  iVar7 = FUN_0053bb80(param_1,1,0);
  iVar2 = FUN_0053bb80(param_1,2,0);
  iVar3 = FUN_0053bb80(param_1,3,0);
  iVar4 = FUN_0053bb80(param_1,0,1);
  iVar4 = iVar6 * 100 + iVar4 * 100;
  iVar6 = FUN_0053bb80(param_1,1,1);
  iVar6 = iVar7 * 100 + iVar6 * 100;
  iVar7 = FUN_0053bb80(param_1,2,1);
  iVar7 = iVar2 * 100 + iVar7 * 100;
  iVar2 = FUN_0053bb80(param_1,3,1);
  iVar2 = iVar3 * 100 + iVar2 * 100;
  if (((*(uint *)((int)this + 0x18) & 0x400) == 0) &&
     ((((0 < iVar4 || (0 < iVar6)) || (0 < iVar7)) || (0 < iVar2)))) {
    *(uint *)((int)this + 0x18) = *(uint *)((int)this + 0x18) | 1;
  }
  iVar3 = FUN_0053bb80(param_1,0,2);
  iVar4 = iVar4 + (iVar3 * 100) / 6;
  iVar3 = FUN_0053bb80(param_1,1,2);
  iVar6 = iVar6 + (iVar3 * 100) / 6;
  iVar3 = FUN_0053bb80(param_1,2,2);
  iVar7 = iVar7 + (iVar3 * 100) / 6;
  iVar3 = FUN_0053bb80(param_1,3,2);
  iVar2 = iVar2 + (iVar3 * 100) / 6;
  if (((0 < iVar4) || (0 < iVar6)) || ((0 < iVar7 || (0 < iVar2)))) {
    if (iVar4 < iVar6) {
LAB_00473c3e:
      if ((iVar6 < iVar7) || (iVar6 < iVar2)) {
LAB_00473c7a:
        if (((iVar7 < iVar4) || (iVar7 < iVar6)) || (iVar7 < iVar2)) {
          if (((iVar4 <= iVar2) && (iVar7 <= iVar2)) && (iVar6 <= iVar2)) {
            iVar6 = FUN_0053bb80(param_1,3,2);
            iVar7 = FUN_0053bb80(param_1,3,1);
            local_10 = FUN_0053bb80(param_1,3,0);
            local_10 = iVar6 + iVar7 + local_10;
            *(int *)((int)this + 0x30) = iVar2;
          }
        }
        else {
          iVar6 = FUN_0053bb80(param_1,2,2);
          iVar2 = FUN_0053bb80(param_1,2,0);
          local_10 = FUN_0053bb80(param_1,2,1);
          local_10 = iVar6 + iVar2 + local_10;
          *(int *)((int)this + 0x30) = iVar7;
        }
      }
      else {
        iVar7 = FUN_0053bb80(param_1,1,1);
        iVar2 = FUN_0053bb80(param_1,1,0);
        local_10 = FUN_0053bb80(param_1,1,2);
        local_10 = iVar7 + iVar2 + local_10;
        *(int *)((int)this + 0x30) = iVar6;
      }
    }
    else {
      if ((iVar4 < iVar7) || (iVar4 < iVar2)) {
        if (iVar4 <= iVar6) goto LAB_00473c3e;
        goto LAB_00473c7a;
      }
      iVar6 = FUN_0053bb80(param_1,0,0);
      iVar7 = FUN_0053bb80(param_1,0,1);
      local_10 = FUN_0053bb80(param_1,0,2);
      local_10 = iVar6 + iVar7 + local_10;
      *(int *)((int)this + 0x30) = iVar4;
    }
    *(int *)((int)this + 0x30) =
         (*(int *)((int)param_1 + 0xdc) * *(int *)((int)this + 0x30)) / local_10;
  }
  iVar6 = FUN_0053bb80(param_1,0,2);
  iVar7 = FUN_0053bb80(param_1,1,2);
  iVar2 = FUN_0053bb80(param_1,2,2);
  iVar4 = FUN_0053bb80(param_1,3,2);
  if ((((*(uint *)((int)this + 0x18) & 0x400) == 0) && (*(int *)((int)this + 0x48) == 0)) &&
     ((0 < iVar6 * 10 || (((0 < iVar7 * 10 || (0 < iVar2 * 10)) || (0 < iVar4 * 10)))))) {
    *(uint *)((int)this + 0x18) = *(uint *)((int)this + 0x18) | 2;
  }
  iVar3 = FUN_0053bb80(param_1,0,0);
  iVar9 = iVar6 * 10 + (iVar3 * 10) / 2;
  iVar6 = FUN_0053bb80(param_1,1,0);
  iVar3 = iVar7 * 10 + (iVar6 * 10) / 2;
  iVar6 = FUN_0053bb80(param_1,2,0);
  iVar7 = iVar2 * 10 + (iVar6 * 10) / 2;
  iVar6 = FUN_0053bb80(param_1,3,0);
  iVar6 = iVar4 * 10 + (iVar6 * 10) / 2;
  if (((iVar9 < 1) && (iVar3 < 1)) && ((iVar7 < 1 && (iVar6 < 1)))) goto LAB_00473ee2;
  if (iVar9 < iVar3) {
LAB_00473e3b:
    if ((iVar3 < iVar7) || (iVar3 < iVar6)) {
LAB_00473e66:
      if (((iVar7 < iVar9) || (iVar7 < iVar3)) || (iVar7 < iVar6)) {
        if (((iVar9 <= iVar6) && (iVar7 <= iVar6)) && (iVar3 <= iVar6)) {
          local_10 = FUN_0053bb80(param_1,3,2);
          iVar7 = FUN_0053bb80(param_1,3,0);
          local_10 = local_10 + iVar7;
          *(int *)((int)this + 0x34) = iVar6;
        }
      }
      else {
        local_10 = FUN_0053bb80(param_1,2,2);
        iVar6 = FUN_0053bb80(param_1,2,0);
        local_10 = local_10 + iVar6;
        *(int *)((int)this + 0x34) = iVar7;
      }
    }
    else {
      local_10 = FUN_0053bb80(param_1,1,0);
      iVar6 = FUN_0053bb80(param_1,1,2);
      local_10 = local_10 + iVar6;
      *(int *)((int)this + 0x34) = iVar3;
    }
  }
  else {
    if ((iVar9 < iVar7) || (iVar9 < iVar6)) {
      if (iVar9 <= iVar3) goto LAB_00473e3b;
      goto LAB_00473e66;
    }
    local_10 = FUN_0053bb80(param_1,0,0);
    iVar6 = FUN_0053bb80(param_1,0,2);
    local_10 = local_10 + iVar6;
    *(int *)((int)this + 0x34) = iVar9;
  }
  *(int *)((int)this + 0x34) =
       (*(int *)((int)param_1 + 0x68) * *(int *)((int)param_1 + 0xdc) * *(int *)((int)this + 0x34))
       / local_10;
LAB_00473ee2:
  *(uint *)((int)this + 0x38) = (uint)(*(int *)((int)param_1 + 0xd4) != 0);
  uVar5 = FUN_0053b860((int)param_1);
  *(undefined4 *)((int)this + 0x1c) = uVar5;
  iVar6 = FUN_0053b870((int)param_1);
  *(int *)((int)this + 0x20) = iVar6;
  *(undefined4 *)((int)this + 0x24) = *(undefined4 *)((int)param_1 + 0x50);
  *(undefined4 *)((int)this + 0x4c) = *(undefined4 *)((int)param_1 + 0x5c);
  iVar7 = *(int *)((int)this + 0x30) * 10;
  *(int *)((int)this + 0x58) = iVar7 / iVar6;
  *(int *)((int)this + 0x5c) = iVar7 / *(int *)((int)this + 0x1c);
  iVar6 = *(int *)((int)param_1 + 0x44);
  if (*(int *)((int)param_1 + 0xc0) < 0x44c) {
    uVar8 = *(uint *)((int)this + 0x18) | 0x10;
  }
  else if (*(int *)((int)param_1 + 0xc0) < 2000) {
    uVar8 = *(uint *)((int)this + 0x18) | 0x20;
  }
  else {
    uVar8 = *(uint *)((int)this + 0x18) | 0x40;
  }
  *(uint *)((int)this + 0x18) = uVar8;
  if (iVar6 == 0) {
    *(undefined4 *)((int)this + 0x28) = 1;
  }
  else {
    *(undefined4 *)((int)this + 0x28) = 2;
  }
  if (*(int *)((int)this + 0x48) != 0) {
    uVar8 = *(uint *)((int)this + 0x18);
    *(uint *)((int)this + 0x18) = uVar8 | 0x200;
    if (((uVar8 & 0x403) == 0) && ((uVar8 & 0x100000) != 0)) {
      *(uint *)((int)this + 0x18) = uVar8 | 0x204;
    }
    if ((*(byte *)((int)this + 0x18) & 1) != 0) {
      *(uint *)((int)this + 0x18) = *(uint *)((int)this + 0x18) | 8;
    }
  }
  if (*(int *)((int)this + 0x44) != 0) {
    *(uint *)((int)this + 0x18) = *(uint *)((int)this + 0x18) | 0x100;
  }
  if (*(int *)((int)this + 0x38) != 0) {
    *(uint *)((int)this + 0x18) = *(uint *)((int)this + 0x18) | 0x800;
  }
  return;
}

